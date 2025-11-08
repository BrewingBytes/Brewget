#!/usr/bin/env node

import { readFileSync, writeFileSync } from "fs";
import { dirname, join } from "path";
import { fileURLToPath } from "url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

/**
 * Parses a changelog markdown file into structured data
 * @param {string} content - The markdown content
 * @returns {Array<Object>} Array of changelog entries
 */
function parseChangelog(content) {
  const entries = [];
  const lines = content.split("\n");
  let inContent = false;

  let currentEntry = null;
  let currentSection = null;

  // Regex patterns for more robust parsing
  const versionHeaderRegex = /^##\s+\[(\d+\.\d+\.\d+)\]\s+-\s+(\d{4}-\d{2}-\d{2})/;
  const sectionHeaderRegex = /^###\s+(Added|Changed|Fixed)/i;
  const bulletPointRegex = /^-\s+(.+)/;

  for (const line of lines) {
    const trimmedLine = line.trim();

    // Skip header lines
    if (
      trimmedLine.startsWith("# Changelog") ||
      trimmedLine.startsWith("All notable changes")
    ) {
      inContent = true;
      continue;
    }

    if (!inContent && trimmedLine === "") {
      continue;
    }

    inContent = true;

    // Match version header
    const versionMatch = versionHeaderRegex.exec(trimmedLine);
    if (versionMatch && versionMatch[1] && versionMatch[2]) {
      if (currentEntry) {
        entries.push(currentEntry);
      }
      currentEntry = {
        version: versionMatch[1],
        date: versionMatch[2],
        changes: { added: [], changed: [], fixed: [] },
      };
      currentSection = null;
      continue;
    }

    // Match section headers
    const sectionMatch = sectionHeaderRegex.exec(trimmedLine);
    if (sectionMatch && sectionMatch[1]) {
      const sectionName = sectionMatch[1].toLowerCase();
      if (
        sectionName === "added" ||
        sectionName === "changed" ||
        sectionName === "fixed"
      ) {
        currentSection = sectionName;
      }
      continue;
    }

    // Match bullet points
    const bulletMatch = bulletPointRegex.exec(trimmedLine);
    if (bulletMatch && bulletMatch[1] && currentEntry && currentSection) {
      const change = bulletMatch[1].trim();
      if (change) {
        currentEntry.changes[currentSection].push(change);
      }
    }
  }

  if (currentEntry) {
    entries.push(currentEntry);
  }

  return entries;
}

// Service names mapping
const services = [
  { file: "frontend-CHANGELOG.md", name: "Frontend" },
  { file: "auth-service-CHANGELOG.md", name: "Auth Service" },
  { file: "settings-service-CHANGELOG.md", name: "Settings Service" },
  { file: "transaction-service-CHANGELOG.md", name: "Transaction Service" },
  { file: "email-service-CHANGELOG.md", name: "Email Service" },
];

const changelogs = [];

for (const service of services) {
  const filePath = join(__dirname, "..", "..", "changelogs", service.file);
  try {
    const content = readFileSync(filePath, "utf-8");
    const entries = parseChangelog(content);
    changelogs.push({
      service: service.name,
      entries,
    });
  } catch (error) {
    console.error(`✗ Failed to parse ${service.file}:`, error);
  }
}

// Write the combined JSON file
const outputPath = join(__dirname, "..", "public", "changelogs.json");
writeFileSync(outputPath, JSON.stringify(changelogs, null, 2));
