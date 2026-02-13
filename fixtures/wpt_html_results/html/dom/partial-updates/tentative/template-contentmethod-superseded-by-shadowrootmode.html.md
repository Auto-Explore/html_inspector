# html/dom/partial-updates/tentative/template-contentmethod-superseded-by-shadowrootmode.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/partial-updates/tentative/template-contentmethod-superseded-by-shadowrootmode.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<meta charset="utf-8" />
<title>HTML partial updates: shadowrootmode supersedes contentmethod</title>
<link rel=help href="https://github.com/WICG/declarative-partial-updates">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<div id="container">
    <section id=outer contentname="d">Unchanged</section>
    <template shadowrootmode="open" contentmethod="append">
        <section id=inner contentname="d">Inside</section>
    </template>
</div>

<script>
test(() => {
    const outer = document.querySelector("#outer");
    assert_equals(outer.textContent, "Unchanged");
    const inner = document.querySelector("#container").shadowRoot.querySelector("#inner");
    assert_not_equals(inner, null);
    assert_equals(inner.textContent, "Inside");
});
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
        "byte_end": 378,
        "byte_start": 368,
        "col": 48,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.section.lacks_heading",
      "message": "Section lacks heading. Consider using “h2”-“h6” elements to add identifying headings to all sections, or else use a “div” element instead for any cases where no heading is needed.",
      "severity": "Warning",
      "span": {
        "byte_end": 497,
        "byte_start": 487,
        "col": 49,
        "line": 11
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/dom/partial-updates/tentative/template-contentmethod-superseded-by-shadowrootmode.html"
}
```
