
/// <reference path='./.sst/platform/config.d.ts' />

export default $config({
  app(input) {
    return {
      name: 'service-mocker',
      removal: 'remove',
      home: 'aws',
      providers: {
        aws: { region: 'us-east-1' }
      },
      stage: input?.stage
    };
  },
  async run() {
    const { stage } = $app;
    const environment = {
      STAGE: stage,
      AWS_REGION: 'us-east-1',
      LOG_LEVEL: process.env.LOG_LEVEL,
      AWS_ACCESS_KEY_ID: process.env.AWS_ACCESS_KEY_ID,
      AWS_SECRET_ACCESS_KEY: process.env.AWS_SECRET_ACCESS_KEY,
      MONGO_URI: process.env.MONGO_URI,
    }

    const api = new sst.aws.Function('api', {
      handler: 'bootstrap',
      bundle: 'target/lambda/api',
      memory: '500 MB',
      timeout: '10 minutes',
      architecture: "arm64",
      url: { cors: true, allowCredentials: true },
      logging: {
        retention: '1 week',
        format: 'json'
      },
      environment: {
        ...environment,
      },
    });

    return {
      url: api.url,
    }
  },
});
