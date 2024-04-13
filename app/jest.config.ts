import type { Config } from '@jest/types';

const config: Config.InitialOptions = {
    preset: 'ts-jest',
    testEnvironment: 'node',
    transform: {
        '^.+\\.ts$': ['ts-jest', { /* ts-jest config goes here in Jest */ }],
    },
    globals: {
        'ts-jest': {
            tsconfig: 'tsconfig.json'
        }
    },
    testTimeout: 30000
};

export default config;
