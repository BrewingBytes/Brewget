#!/usr/bin/env node

/**
 * All-in-one script to set up and deploy transaction feature components
 * Run this script from the frontend directory: node complete-transaction-setup.mjs
 * 
 * This script:
 * 1. Creates required directories
 * 2. Deploys component files from /tmp to correct locations
 * 3. Reports success/failure
 */

import { copyFileSync, existsSync, mkdirSync } from 'fs';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

console.log('========================================');
console.log('Transaction Feature Setup & Deployment');
console.log('========================================\n');

// Step 1: Create directories
console.log('Step 1: Creating directories...\n');

const directories = [
  'src/components/transactions',
  'src/components/categories',
];

let dirErrors = false;

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
      dirErrors = true;
    }
  }
});

if (dirErrors) {
  console.error('\n✗ Failed to create some directories. Aborting.\n');
  process.exit(1);
}

console.log('\n✓ All directories created successfully!\n');

// Step 2: Deploy component files
console.log('Step 2: Deploying component files...\n');

const fileMappings = [
  {
    source: '/tmp/TransactionCard.vue',
    dest: 'components/transactions/TransactionCard.vue',
    name: 'TransactionCard',
  },
  {
    source: '/tmp/TransactionList.vue',
    dest: 'components/transactions/TransactionList.vue',
    name: 'TransactionList',
  },
  {
    source: '/tmp/TransactionCreateDialog.vue',
    dest: 'components/transactions/TransactionCreateDialog.vue',
    name: 'TransactionCreateDialog',
  },
  {
    source: '/tmp/TransactionEditDialog.vue',
    dest: 'components/transactions/TransactionEditDialog.vue',
    name: 'TransactionEditDialog',
  },
  {
    source: '/tmp/TransactionDeleteDialog.vue',
    dest: 'components/transactions/TransactionDeleteDialog.vue',
    name: 'TransactionDeleteDialog',
  },
  {
    source: '/tmp/CategoryManageDialog.vue',
    dest: 'components/categories/CategoryManageDialog.vue',
    name: 'CategoryManageDialog',
  },
  {
    source: '/tmp/TransactionsView.vue',
    dest: 'views/TransactionsView.vue',
    name: 'TransactionsView',
  },
];

let successCount = 0;
let failCount = 0;
const failedFiles = [];

fileMappings.forEach(({ source, dest, name }) => {
  const destPath = join(__dirname, 'src', dest);
  
  try {
    if (!existsSync(source)) {
      console.error(`✗ ${name}: Source file not found at ${source}`);
      failCount++;
      failedFiles.push(name);
      return;
    }

    copyFileSync(source, destPath);
    console.log(`✓ ${name}: Deployed to src/${dest}`);
    successCount++;
  } catch (error) {
    console.error(`✗ ${name}: Failed to deploy - ${error.message}`);
    failCount++;
    failedFiles.push(name);
  }
});

// Summary
console.log('\n========================================');
console.log('Deployment Summary');
console.log('========================================\n');
console.log(`✓ Successfully deployed: ${successCount} file(s)`);

if (failCount > 0) {
  console.log(`✗ Failed to deploy: ${failCount} file(s)`);
  console.log('\nFailed files:');
  failedFiles.forEach(file => console.log(`  - ${file}`));
  console.log('\n');
  process.exit(1);
}

console.log('\n✅ Transaction feature setup complete!\n');
console.log('Next steps:');
console.log('  1. Run "bun install" to ensure dependencies are up to date');
console.log('  2. Run "bun format" to format all code');
console.log('  3. Run "bun type-check" to verify TypeScript compilation');
console.log('  4. Run "bun build" to build the project');
console.log('  5. Test the transaction features in your browser\n');
