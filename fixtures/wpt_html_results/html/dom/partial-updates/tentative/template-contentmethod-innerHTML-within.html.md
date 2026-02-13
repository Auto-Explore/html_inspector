# html/dom/partial-updates/tentative/template-contentmethod-innerHTML-within.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/partial-updates/tentative/template-contentmethod-innerHTML-within.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<meta charset="utf-8" />
<title>HTML partial updates: patching via innerHTML</title>
<link rel=help href="https://github.com/WICG/declarative-partial-updates">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="container"></div>
<script>
test(() => {
    const container = document.getElementById("container");
    container.innerHTML = `
        <div id="placeholder" contentname=p>Old content in innerHTML</div>
        <template contentmethod="replace-children">
            <div contentname="p">New content</div>
        </template>`;
    const placeholder = container.querySelector("div");
    assert_equals(placeholder.id, "placeholder");
    assert_equals(placeholder.textContent, "New content");
    assert_equals(container.querySelector("template"), null);
}, "<template contentmethod> in innerHTML patching inner element");
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
  "source_name": "html/dom/partial-updates/tentative/template-contentmethod-innerHTML-within.html"
}
```
