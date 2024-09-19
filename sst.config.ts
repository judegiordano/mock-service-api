/// <reference path='./.sst/platform/config.d.ts' />

const FIVE_MINUTES_IN_SECONDS = 60 * 5;

const domain = "muckapp.com"

export default $config({
  app(input) {
    return {
      name: "service-mocker",
      removal: "remove",
      home: "aws",
      providers: {
        aws: { region: "us-east-1" },
        eks: true,
      },
      stage: input?.stage,
    };
  },
  async run() {
    const { stage } = $app;
    const handler = "bootstrap";
    const runtime = "provided.al2023";
    const logging = { retention: "1 week", format: "json" };
    const environment = {
      STAGE: stage,
      LOG_LEVEL: process.env.LOG_LEVEL,
      MONGO_URI: process.env.MONGO_URI,
    };

    const api = new sst.aws.Function("api", {
      handler,
      runtime,
      bundle: "target/lambda/api",
      memory: "! GB",
      timeout: "10 minutes",
      architecture: "arm64",
      url: { cors: true, allowCredentials: true },
      logging,
      environment,
    });

    // TODO: register domain
    // const router = new sst.aws.Router("router", {
    //   invalidation: false,
    //   transform: {
    //     cachePolicy: {
    //       defaultTtl: FIVE_MINUTES_IN_SECONDS,
    //       minTtl: FIVE_MINUTES_IN_SECONDS,
    //       maxTtl: FIVE_MINUTES_IN_SECONDS,
    //     }
    //   },
    //   domain: {
    //     name: `api.${domain}`,
    //     redirects: [`www.api.${domain}`]
    //   },
    //   routes: { '/*': api.url }
    // })

    return {
      url: api.url,
      // domain: router.url
    };
  },
});
