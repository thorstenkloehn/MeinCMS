const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

const rootDir = path.resolve(__dirname, '..');
const agentsMdPath = path.join(rootDir, '.agents', 'AGENTS.md');
const skillsDir = path.join(rootDir, '.agents', 'skills');
const docsSrcDir = path.join(rootDir, 'docs', 'src');
const docsSkillsDir = path.join(docsSrcDir, 'skills');

console.log('Synchronizing AGENTS.md and Skills into docs/src...');

// 1. Copy AGENTS.md -> docs/src/agents.md
if (fs.existsSync(agentsMdPath)) {
  const content = fs.readFileSync(agentsMdPath, 'utf8');
  fs.writeFileSync(path.join(docsSrcDir, 'agents.md'), content);
  console.log('  -> Updated docs/src/agents.md');
}

// 2. Prepare docs/src/skills directory
if (!fs.existsSync(docsSkillsDir)) {
  fs.mkdirSync(docsSkillsDir, { recursive: true });
}

// Helper to parse YAML frontmatter if present
function parseSkillFile(filePath) {
  const content = fs.readFileSync(filePath, 'utf8');
  let name = '';
  let description = '';
  let body = content;

  if (content.startsWith('---')) {
    const parts = content.split('---');
    if (parts.length >= 3) {
      const frontmatter = parts[1];
      body = parts.slice(2).join('---').trim();
      const nameMatch = frontmatter.match(/name:\s*(.+)/);
      const descMatch = frontmatter.match(/description:\s*(.+)/);
      if (nameMatch) name = nameMatch[1].trim();
      if (descMatch) description = descMatch[1].trim();
    }
  }

  return { name, description, body };
}

// Read all skills from .agents/skills
const skillsList = [];
if (fs.existsSync(skillsDir)) {
  const entries = fs.readdirSync(skillsDir, { withFileTypes: true });
  for (const entry of entries) {
    if (entry.isDirectory()) {
      const skillName = entry.name;
      const skillFile = path.join(skillsDir, skillName, 'SKILL.md');
      if (fs.existsSync(skillFile)) {
        const { name, description, body } = parseSkillFile(skillFile);
        const displayName = name || skillName;
        const skillDocPath = path.join(docsSkillsDir, `${skillName}.md`);
        
        let formattedDoc = body;
        if (description && !formattedDoc.includes(description)) {
          formattedDoc = `> **Beschreibung:** ${description}\n\n` + formattedDoc;
        }
        fs.writeFileSync(skillDocPath, formattedDoc);
        console.log(`  -> Updated docs/src/skills/${skillName}.md`);

        skillsList.push({
          id: skillName,
          title: displayName,
          description: description
        });
      }
    }
  }
}

// 3. Create docs/src/skills/README.md
let skillsOverview = '# Agent Skills Overview\n\nÜbersicht aller verfügbaren Subagent Skills:\n\n';
for (const skill of skillsList) {
  skillsOverview += `- [**${skill.title}**](./${skill.id}.md): ${skill.description}\n`;
}
fs.writeFileSync(path.join(docsSkillsDir, 'README.md'), skillsOverview);
console.log('  -> Updated docs/src/skills/README.md');

// 4. Update docs/src/SUMMARY.md
let summaryContent = `# Summary

- [1. Installation & Setup](./installation.md)
- [2. Administrator-Handbuch](./administrator_handbuch.md)
- [3. System-Architektur, Bausteine & Design Patterns](./architektur_design_patterns.md)
- [4. Externe Bibliotheken & Crates](./externe_bibliotheken.md)
- [5. C-API / C-ABI, Conan, NPM & PIP Paketmanager](./c_api_conan_npm.md)
- [6. Sicherheits-Handbuch: Ubuntu, Rust & NPM Ökosystem](./ubuntu_security.md)
- [7. KI-Agenten Konfiguration (AGENTS.md)](./agents.md)
- [8. KI-Agenten, Subagenten, Prompt-Sicherheit & Praxis-Handbuch Vibe Coding](./ai_agent_security_vibe_coding.md)
- [9. AGY CLI & IDE: Praxis-Workflow, Einstellungen & Token-Optimierung](./agy_praxis_performance.md)
  - [Schritt-für-Schritt Setup: AGY CLI & IDE](./agy_workflow/cli_ide_setup.md)
  - [Tutorial: KI-Geschwindigkeit & Token-Spar-Optimierung](./agy_workflow/token_speed_optimization.md)
  - [Mensch & KI als Team (Augmented Engineering)](./agy_workflow/human_ai_collaboration.md)
- [10. Agent Skills](./skills/README.md)
`;

for (const skill of skillsList) {
  summaryContent += `  - [${skill.title}](./skills/${skill.id}.md)\n`;
}

summaryContent += `- [11. Impressum](./Impressum.md)
- [12. Datenschutz](./Datenschschutz.md)
`;

fs.writeFileSync(path.join(docsSrcDir, 'SUMMARY.md'), summaryContent);
console.log('  -> Updated docs/src/SUMMARY.md');

// 5. Build mdBook
console.log('Building mdBook documentation...');
try {
  execSync('mdbook build docs', { stdio: 'inherit', cwd: rootDir });
  console.log('mdBook build successful!');
} catch (err) {
  console.error('Failed to build mdBook:', err);
  process.exit(1);
}
