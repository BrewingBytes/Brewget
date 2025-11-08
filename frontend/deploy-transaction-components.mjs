#!/usr/bin/env node

/**
 * Deployment script to move component files from /tmp to their correct locations
 * Run this script after running setup-transaction-dirs.mjs
 * 
 * Usage: node deploy-transaction-components.mjs
 */

import { copyFileSync, existsSync } from 'fs';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

// Map of source files (in /tmp) to destination paths (relative to frontend/src)
const fileMappings = [
  {
    source: '/tmp/TransactionCard.vue',
    dest: 'components/transactions/TransactionCard.vue',
  },
  {
    source: '/tmp/TransactionList.vue',
    dest: 'components/transactions/TransactionList.vue',
  },
  {
    source: '/tmp/TransactionCreateDialog.vue',
    dest: 'components/transactions/TransactionCreateDialog.vue',
  },
  {
    source: '/tmp/TransactionEditDialog.vue',
    dest: 'components/transactions/TransactionEditDialog.vue',
  },
  {
    source: '/tmp/TransactionDeleteDialog.vue',
    dest: 'components/transactions/TransactionDeleteDialog.vue',
  },
  {
    source: '/tmp/CategoryManageDialog.vue',
    dest: 'components/categories/CategoryManageDialog.vue',
  },
  {
    source: '/tmp/TransactionsView.vue',
    dest: 'views/TransactionsView.vue',
  },
];

console.log('Deploying transaction components...\n');

let successCount = 0;
let failCount = 0;

fileMappings.forEach(({ source, dest }) => {
  const destPath = join(__dirname, 'src', dest);
  
  try {
    if (!existsSync(source)) {
      console.error(`✗ Source file not found: ${source}`);
      failCount++;
      return;
    }

    copyFileSync(source, destPath);
    console.log(`✓ Deployed: ${dest}`);
    successCount++;
  } catch (error) {
    console.error(`✗ Failed to deploy ${dest}:`, error.message);
    failCount++;
  }
});

console.log(`\n✓ Deployment complete!`);
console.log(`  Success: ${successCount} files`);
if (failCount > 0) {
  console.log(`  Failed: ${failCount} files`);
  process.exit(1);
}

console.log('\nNext steps:');
console.log('  1. Run "bun format" to format the new files');
console.log('  2. Run "bun build" to verify everything compiles');
console.log('  3. Test the transaction features in the browser\n');
