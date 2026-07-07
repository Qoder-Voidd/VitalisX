import Image from "next/image";
import Link from "next/link";
import FeatureStrip from "./FeatureStrip";

export default function Hero() {
  return (
    // Full-viewport-height hero section; pt-24 offsets a fixed/sticky navbar
    // of that height so content doesn't render underneath it.
    <section className="relative flex min-h-screen flex-col bg-black pt-24">
      {/* Two-column layout on large screens: copy on the left (1.2fr),
          hero image on the right (0.8fr). Stacks to a single column on
          smaller viewports since no grid-cols override exists below lg. */}
      <div className="mx-auto grid w-full max-w-7xl flex-1 grid-cols-1 items-center gap-8 px-6 lg:grid-cols-[1.2fr_0.8fr] lg:gap-12">
        {/* Left column: headline, subcopy, and CTAs */}
        <div className="flex flex-col gap-5">
          {/* Main headline — key action words ("Learn", "Stellar") are
              italicized and colored with the brand's primary color for emphasis */}
          <h1 className="text-3xl font-light leading-tight tracking-tight text-white sm:text-4xl lg:text-[3.25rem] lg:leading-[1.15]">
            <span className="italic text-primary">Learn</span>. Trade. Connect.
            <br />
            Powered by AI on{" "}
            <span className="italic text-primary">Stellar</span>.
          </h1>

          {/* Supporting subcopy summarizing the product's three pillars:
              AI learning, social insights, on-chain trading */}
          <p className="text-base leading-relaxed text-white sm:text-lg">
            VitalisX is an all-in-one Web3 academy combining AI-powered
            learning, social crypto insights, and real on-chain trading — built
            on Stellar.
          </p>

          {/* Primary + secondary CTA buttons, anchor-linking to page sections */}
          <div className="flex flex-wrap gap-3 pt-1">
            {/* Primary CTA — solid brand-color pill button */}
            <Link
              href="#get-started"
              className="rounded-full bg-primary px-5 py-2 text-sm font-medium text-white transition-opacity hover:opacity-90"
            >
              Get Started
            </Link>
            {/* Secondary CTA — outlined/ghost style so it doesn't compete
                visually with the primary button */}
            <Link
              href="#learn-more"
              className="rounded-full border border-white/30 px-5 py-2 text-sm font-medium text-white transition-colors hover:border-white hover:bg-white/5"
            >
              Learn More
            </Link>
          </div>
        </div>

        {/* Right column: hero illustration/assistant image.
            Centered on mobile, right-aligned on large screens to balance
            the left-heavy text column. */}
        <div className="relative flex items-center justify-center lg:justify-end">
          <Image
            src="/images/heroImage.png"
            alt="VitalisX assistant"
            width={500}
            height={500}
            // priority: preloads this above-the-fold image for faster LCP
            // unoptimized: skips Next.js image optimization pipeline
            // (e.g. for a static/exported build, or if the source is
            // already optimized/served from elsewhere)
            className="h-auto w-full max-w-sm object-contain lg:max-w-md xl:max-w-lg"
            priority
            unoptimized
          />
        </div>
      </div>

      {/* Secondary content strip (e.g. logos, stats, or feature highlights)
          rendered below the main hero grid, still inside the same section */}
      <FeatureStrip />
    </section>
  );
}