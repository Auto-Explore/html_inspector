# html/dom/partial-updates/tentative/template-contentmethod-innerHTML-outside.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/partial-updates/tentative/template-contentmethod-innerHTML-outside.html",
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
<div id="placeholder" contentname="p">Old content</div>
<div id="container"></div>
<script>
test(() => {
    const placeholder = document.getElementById("placeholder");
    const container = document.getElementById("container");
    assert_equals(placeholder.textContent, "Old content");
    container.innerHTML = "<template contentmethod=replace><div contentname=p>New content</div></template>";
    assert_equals(placeholder.textContent, "Old content");
    // The <template> element should not be inserted, so no child nodes.
    assert_false(container.hasChildNodes(), "template should not attach");
}, "<template contentmethod> in innerHTML should not patch outer element");
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
  "source_name": "html/dom/partial-updates/tentative/template-contentmethod-innerHTML-outside.html"
}
```
