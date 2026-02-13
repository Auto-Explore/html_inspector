# html/dom/partial-updates/tentative/template-contentmethod-innerHTML-ambiguous.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/partial-updates/tentative/template-contentmethod-innerHTML-ambiguous.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<meta charset="utf-8" />
<title>HTML partial updates: patching via innerHTML with ambiguous target</title>
<link rel=help href="https://github.com/WICG/declarative-partial-updates">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="container"></div>
<div id="placeholder" contentname="placeholder">Old content in light DOM</div>
<script>
// The contentname "placeholder" appears both after the container on which innerHTML is
// set, and in created fragment. Which element should be updated depends on the
// details of how this is spec'd.
test(() => {
    const container = document.getElementById("container");
    const outerPlaceholder = document.getElementById("placeholder");
    container.innerHTML = `<div id="placeholder" contentname="placeholder">Old content in innerHTML</div><template contentmethod="replace-children"><div contentname="placeholder">New content</div></template>`;
    const innerPlaceholder = container.firstChild;
    assert_equals(innerPlaceholder.id, "placeholder");
    // patches apply inside the fragment
    assert_equals(outerPlaceholder.textContent, "Old content in light DOM");
    assert_equals(innerPlaceholder.textContent, "New content");
}, "<template patchfor> in innerHTML patching inner element");
</script>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/dom/partial-updates/tentative/template-contentmethod-innerHTML-ambiguous.html"
}
```
