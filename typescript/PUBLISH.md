# NPM Publish Checklist

## ✅ Pre-Publish Verification

### 1. Install Dependencies
```bash
npm install
```

### 2. Build TypeScript
```bash
npm run build
```
- [ ] Compiles without errors
- [ ] `dist/` directory created
- [ ] `.d.ts` files generated

### 3. Run Tests
```bash
npm test
```
- [ ] All tests pass
- [ ] No test failures

### 4. Run Linter
```bash
npm run lint
```
- [ ] No linting errors
- [ ] Code style consistent

### 5. Verify Package Contents
```bash
npm pack --dry-run
```
- [ ] Only includes necessary files
- [ ] `dist/` included
- [ ] `src/` excluded
- [ ] `node_modules/` excluded

### 6. Test Local Install
```bash
npm pack
npm install -g truthlinked-sdk-*.tgz
```
- [ ] Installs successfully
- [ ] Can import in test project

### 7. Verify Package.json
- [ ] Name: `@truthlinked/sdk`
- [ ] Version: Correct semver
- [ ] Main: `dist/index.js`
- [ ] Types: `dist/index.d.ts`
- [ ] Files: `["dist", "README.md", "LICENSE"]`
- [ ] Repository URL set
- [ ] License: MIT

### 8. Documentation Check
- [ ] README.md complete
- [ ] Examples work
- [ ] API documented
- [ ] Security notes included

### 9. Security Audit
```bash
npm audit
```
- [ ] No high/critical vulnerabilities
- [ ] Dependencies up to date

## 🚀 Publishing

### First Time Setup
```bash
npm login
```

### Publish to npm
```bash
npm publish --access public
```

### Verify Published Package
```bash
npm view @truthlinked/sdk
```

### Test Installation
```bash
npm install @truthlinked/sdk
```

## 📋 Post-Publish

- [ ] Tag release in git: `git tag v1.0.0`
- [ ] Push tags: `git push --tags`
- [ ] Create GitHub release
- [ ] Update documentation site
- [ ] Announce release

## 🔄 Version Updates

### Patch (1.0.0 → 1.0.1)
```bash
npm version patch
npm publish
```

### Minor (1.0.0 → 1.1.0)
```bash
npm version minor
npm publish
```

### Major (1.0.0 → 2.0.0)
```bash
npm version major
npm publish
```

## ⚠️ Troubleshooting

### Build Fails
- Check TypeScript version
- Verify tsconfig.json
- Clear `dist/` and rebuild

### Tests Fail
- Check Node.js version (>=18)
- Verify dependencies installed
- Check test environment

### Publish Fails
- Verify npm login
- Check package name availability
- Verify version not already published

## 📞 Support

- Issues: https://github.com/truth-linked/truthlinked-sdk/issues
- Email: support@truthlinked.org
