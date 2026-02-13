# html/dom/partial-updates/tentative/template-contentmethod-shadow.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/partial-updates/tentative/template-contentmethod-shadow.html",
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
        <section id="placeholder" contentname="p">Old content in shadow DOM</section>
        <template contentmethod="replace-children"><section contentname="p">New content</template>
    </template>
</div>
<script>
test(() => {
    assert_equals(document.querySelector("#placeholder").innerText, "Old content in light DOM");
    const {shadowRoot} = document.querySelector("#container");
    assert_equals(shadowRoot.querySelector("#placeholder").innerText, "New content");
    assert_equals(shadowRoot.querySelector("template[patchfor=placeholder]"), null);
}, "<template contentmethod> inside a <template shadowrootmode> should apply directly to its target");

</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.section.lacks_heading",
      "message": "Section lacks heading. Consider using “h2”-“h6” elements to add identifying headings to all sections, or else use a “div” element instead for any cases where no heading is needed.",
      "severity": "Warning",
      "span": {
        "byte_end": 498,
        "byte_start": 488,
        "col": 76,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.section.lacks_heading",
      "message": "Section lacks heading. Consider using “h2”-“h6” elements to add identifying headings to all sections, or else use a “div” element instead for any cases where no heading is needed.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/dom/partial-updates/tentative/template-contentmethod-shadow.html"
}
```
