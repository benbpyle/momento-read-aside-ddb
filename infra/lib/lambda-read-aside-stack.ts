import * as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';
import { TableConstruct } from './constructs/table-construct';
import { LambdaConstruct } from './constructs/lambda-construct';

export class LambdaReadAsideStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const t = new TableConstruct(this, 'TableConstruct');
    new LambdaConstruct(this, 'LambdaConstruct', t.table);

  }
}
