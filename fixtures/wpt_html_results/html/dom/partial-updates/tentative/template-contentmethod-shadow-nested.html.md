# html/dom/partial-updates/tentative/template-contentmethod-shadow-nested.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/partial-updates/tentative/template-contentmethod-shadow-nested.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<meta charset="utf-8" />
<title>HTML partial updates: patching inside a declarative shadow tree</title>
<link rel=help href="https://github.com/WICG/declarative-partial-updates">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<div id="placeholder">Old content in light DOM</div>
<div id="container">
    <template shadowrootmode="open">
        <div>
            <div id="placeholder" contentname="d">Old content in shadow DOM</div>
            <template contentmethod="replace-children"><div contentname=d>New content</div></template>
        </div>
    </template>
</div>
<script>
test(() => {
    assert_equals(document.querySelector("#placeholder").innerText, "Old content in light DOM");
    const {shadowRoot} = document.querySelector("#container");
    assert_equals(shadowRoot.querySelector("#placeholder").innerText, "New content");
    assert_equals(shadowRoot.querySelector("template[patchfor=placeholder]"), null);
}, "<template contentmethod> inside a <template shadowrootmode><div> should apply directly to its target");

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
  "source_name": "html/dom/partial-updates/tentative/template-contentmethod-shadow-nested.html"
}
```
