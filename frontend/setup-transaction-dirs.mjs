#!/usr/bin/env node

/**
 * Setup script to create necessary directories for transaction feature
 * Run this script from the frontend directory with: node setup-transaction-dirs.mjs
 * 
 * This script creates the required directory structure for the transaction
 * components before the component files are added to the repository.
 */

import { mkdirSync } from 'fs';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const directories = [
  'src/components/transactions',
  'src/components/categories',
];

console.log('Creating directories for transaction feature...\n');

let hasErrors = false;

directories.forEach(dir => {
  const fullPath = join(__dirname, dir);
  try {
    mkdirSync(fullPath, { recursive: true });
    console.log(`✓ Created: ${dir}`);
  } catch (error) {
    if (error.code === 'EEXIST') {
      console.log(`✓ Already exists: ${dir}`);
    } else {
      console.error(`✗ Failed to create ${dir}:`, error.message);
      hasErrors = true;
    }
  }
});

if (hasErrors) {
  console.error('\n✗ Some directories could not be created.');
  process.exit(1);
}

console.log('\n✓ All directories created successfully!');
console.log('\nNext steps:');
console.log('  1. The component files will be added in the next commit');
console.log('  2. Run "bun install" to ensure dependencies are up to date');
console.log('  3. Run "bun format" to format the code');
console.log('  4. Run "bun build" to verify everything compiles\n');
