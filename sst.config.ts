
/// <reference path='./.sst/platform/config.d.ts' />

const ONE_MINUTE_IN_SECONDS = 60;
const FIVE_MINUTES_IN_SECONDS = ONE_MINUTE_IN_SECONDS * 5;
const domain = 'discitapp.com'

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
      LOG_LEVEL: process.env.LOG_LEVEL,
      MONGO_URI: process.env.MONGO_URI,
    }

    const api = new sst.aws.Function('api', {
      handler: 'bootstrap',
      runtime: 'provided.al2023',
      bundle: 'target/lambda/api',
      memory: '2 GB',
      timeout: '10 minutes',
      architecture: 'arm64',
      url: {
        cors: {
          allowCredentials: true
        }
      },
      logging: {
        retention: '1 week',
        format: 'json'
      },
      environment,
    });

    const vpc = new sst.aws.Vpc("ServiceMockerVpc", {
      nat: "ec2",
      bastion: true,
    });
    const cluster = new sst.aws.Cluster("ServiceMockerCluster", { vpc });
    const service = cluster.addService("ServiceMockerService", {
      scaling: { min: 1, max: 1 },
      image: {
        context: "./",
        dockerfile: "Dockerfile",
      },
      logging: { retention: "1 week" },
      memory: "2 GB",
      cpu: "1 vCPU",
      loadBalancer: {
        // domain: "",
        ports: [
          { listen: "80/http" },
          // { listen: "443/https", forward: "80/http" },
        ],
      },
      environment: {
        ...environment,
        PORT: "80"
      },
    });
    const router = new sst.aws.Router('ServiceMockerRouter', {
      invalidation: false,
      transform: {
        cachePolicy: {
          defaultTtl: ONE_MINUTE_IN_SECONDS,
          minTtl: ONE_MINUTE_IN_SECONDS,
          maxTtl: ONE_MINUTE_IN_SECONDS,
        }
      },
      domain: {
        name: `api.${domain}`,
        redirects: [`www.api.${domain}`]
      },
      routes: { '/*': service.url }
    });

    return {
      url: api.url,
      service: service.url,
      cloudfront: router.url
    }
  },
});
