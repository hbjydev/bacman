// @ts-check
import ntm from 'next-transpile-modules';

/**
 * Run `build` or `dev` with `SKIP_ENV_VALIDATION` to skip env validation.
 * This is especially useful for Docker builds.
 */
!process.env.SKIP_ENV_VALIDATION && (await import("./src/env.mjs"));

const withTM = ntm(
  [
    '@patternfly/react-core',
    '@patternfly/react-icons',
    '@patternfly/react-styles',
    '@patternfly/react-charts',
    // '@patternfly/react-table',
    // '@patternfly/react-tokens',
  ],
  { debug: false }
);

/** @type {import("next").NextConfig} */
const config = withTM({
  reactStrictMode: true,

  /**
   * If you have the "experimental: { appDir: true }" setting enabled, then you
   * must comment the below `i18n` config out.
   *
   * @see https://github.com/vercel/next.js/issues/41980
   */
  i18n: {
    locales: ["en"],
    defaultLocale: "en",
  },
});
export default config;
