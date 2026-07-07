import { Injectable, UnauthorizedException, Logger } from '@nestjs/common';
import { JwtService as NestJwtService } from '@nestjs/jwt';
import { ConfigService } from '@nestjs/config';
import { InjectRepository } from '@nestjs/typeorm';
import { DataSource, Repository } from 'typeorm';
import { createHash, randomUUID as uuidv4 } from 'crypto';
import { RefreshToken } from '../entities/refresh-token.entity';
import { User } from '../entities/user.entity';
import { AuditService } from '../../audit/audit.service';
import { AuditEvent } from '../../audit/audit.event';

export interface JwtPayload {
  sub: string; // user id
  walletId?: string;
  iat?: number;
  exp?: number;
}

export interface RefreshTokenResult {
  token: string;
  id: string;
  expiresAt: Date;
}

export interface RefreshedTokens {
  accessToken: string;
  newRefreshToken: string;
}

@Injectable()
export class JwtAuthService {
  private readonly logger = new Logger(JwtAuthService.name);

  constructor(
    private readonly jwtService: NestJwtService,
    private readonly configService: ConfigService,
    private readonly dataSource: DataSource,
    @InjectRepository(RefreshToken)
    private readonly refreshTokenRepository: Repository<RefreshToken>,
    @InjectRepository(User)
    private readonly userRepository: Repository<User>,
    private readonly auditService: AuditService,
  ) {}

  /**
   * Signs a short-lived access token. Issuer/audience are set explicitly so
   * tokens minted by this service can't be silently accepted by another
   * service sharing the same secret.
   */
  async generateAccessToken(userId: string, walletId?: string): Promise<string> {
    const payload: JwtPayload = {
      sub: userId,
      walletId,
    };

    return this.jwtService.sign(payload, {
      expiresIn: this.configService.get<string>('JWT_ACCESS_EXPIRATION', '15m'),
      issuer: this.configService.get<string>('JWT_ISSUER', 'cadence-api'),
      audience: this.configService.get<string>('JWT_AUDIENCE', 'cadence-client'),
    });
  }

  /**
   * Generates a refresh token. The raw token is returned to the caller
   * exactly once and is never stored — only its SHA-256 hash is persisted,
   * so a database leak alone can't be used to mint new sessions.
   */
  async generateRefreshToken(userId: string, familyId?: string): Promise<RefreshTokenResult> {
    const rawToken = uuidv4();
    const tokenHash = this.hashToken(rawToken);

    const expirationDays = this.configService.get<number>('JWT_REFRESH_EXPIRATION_DAYS', 7);
    const expiresAt = new Date();
    expiresAt.setDate(expiresAt.getDate() + expirationDays);

    const refreshToken = this.refreshTokenRepository.create({
      tokenHash,
      userId,
      familyId: familyId ?? uuidv4(),
      expiresAt,
      revoked: false,
    });

    const saved = await this.refreshTokenRepository.save(refreshToken);

    await this.auditService.logAction('REFRESH_TOKEN_CREATED', userId, saved.id, {
      expiresAt: saved.expiresAt,
    });

    // Return the raw (unhashed) token to the caller — this is the only time it exists in plaintext.
    return {
      token: rawToken,
      id: saved.id,
      expiresAt: saved.expiresAt,
    };
  }

  async validateAccessToken(token: string): Promise<JwtPayload> {
    try {
      const payload = this.jwtService.verify(token, {
        issuer: this.configService.get<string>('JWT_ISSUER', 'cadence-api'),
        audience: this.configService.get<string>('JWT_AUDIENCE', 'cadence-client'),
      });
      return payload as JwtPayload;
    } catch (error) {
      throw new UnauthorizedException('Invalid or expired access token');
    }
  }

  /**
   * Rotates a refresh token. Runs inside a transaction so the revoke +
   * re-issue is atomic, and detects reuse of an already-revoked token
   * (a strong signal of theft) by revoking the entire token family.
   */
  async refreshAccessToken(refreshToken: string): Promise<RefreshedTokens> {
    const tokenHash = this.hashToken(refreshToken);

    const tokenRecord = await this.refreshTokenRepository.findOne({
      where: { tokenHash },
      relations: ['user'],
    });

    if (!tokenRecord) {
      throw new UnauthorizedException('Invalid refresh token');
    }

    if (tokenRecord.revoked) {
      // Reuse of a revoked token likely means it was stolen and already
      // used by someone else — kill the whole family, not just this token.
      this.logger.warn(
        `Refresh token reuse detected for user ${tokenRecord.userId}, family ${tokenRecord.familyId}`,
      );
      await this.revokeTokenFamily(tokenRecord.familyId);
      await this.auditService.logAction(
        'REFRESH_TOKEN_REUSE_DETECTED',
        tokenRecord.userId,
        tokenRecord.id,
      );
      throw new UnauthorizedException('Refresh token has been revoked');
    }

    if (new Date() > tokenRecord.expiresAt) {
      throw new UnauthorizedException('Refresh token expired');
    }

    if (!tokenRecord.user?.isActive) {
      throw new UnauthorizedException('User account is inactive');
    }

    return this.dataSource.transaction(async (manager) => {
      await manager.update(
        RefreshToken,
        { id: tokenRecord.id },
        { revoked: true, revokedAt: new Date() },
      );

      const accessToken = await this.generateAccessToken(tokenRecord.userId);
      const newRefreshTokenData = await this.generateRefreshToken(
        tokenRecord.userId,
        tokenRecord.familyId,
      );

      await this.auditService.logAction(
        'ACCESS_TOKEN_REFRESHED',
        tokenRecord.userId,
        tokenRecord.id,
      );

      return {
        accessToken,
        newRefreshToken: newRefreshTokenData.token,
      };
    });
  }

  async revokeRefreshToken(tokenId: string): Promise<void> {
    const tokenRecord = await this.refreshTokenRepository.findOne({ where: { id: tokenId } });

    if (!tokenRecord) {
      throw new UnauthorizedException('Refresh token not found');
    }

    await this.refreshTokenRepository.update(
      { id: tokenId },
      { revoked: true, revokedAt: new Date() },
    );

    await this.auditService.logAction('REFRESH_TOKEN_REVOKED', tokenRecord.userId, tokenId);
  }

  async revokeAllUserRefreshTokens(userId: string): Promise<void> {
    await this.refreshTokenRepository.update(
      { userId, revoked: false },
      { revoked: true, revokedAt: new Date() },
    );

    await this.auditService.logAction('ALL_REFRESH_TOKENS_REVOKED', userId, userId);
  }

  private async revokeTokenFamily(familyId: string): Promise<void> {
    await this.refreshTokenRepository.update(
      { familyId, revoked: false },
      { revoked: true, revokedAt: new Date() },
    );
  }

  async getUserFromToken(token: string): Promise<User> {
    const payload = await this.validateAccessToken(token);
    const user = await this.userRepository.findOne({
      where: { id: payload.sub },
    });

    if (!user) {
      throw new UnauthorizedException('User not found');
    }

    if (!user.isActive) {
      throw new UnauthorizedException('User account is inactive');
    }

    return user;
  }

  private hashToken(token: string): string {
    return createHash('sha256').update(token).digest('hex');
  }
}