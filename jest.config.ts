import type { Config } from 'jest';

const config: Config = {
	verbose: true,
	preset: 'solid-jest/preset/browser',
	testEnvironment: 'jsdom'
};

export default config;
