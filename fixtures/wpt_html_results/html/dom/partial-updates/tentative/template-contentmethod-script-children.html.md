# html/dom/partial-updates/tentative/template-contentmethod-script-children.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/partial-updates/tentative/template-contentmethod-script-children.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<meta charset="utf-8" />
<title>HTML partial updates - a script inside a patch should execute</title>
<link rel=help href="https://github.com/WICG/declarative-partial-updates">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<section id="placeholder" contentname="p">
    <script>
    window.state = "init";
    </script>
</section>
<template contentmethod="replace-children"><section contentname=p><script>window.state = 'patched';</script></section></template>
<script>
test(() => {
    assert_not_equals(document.querySelector("#placeholder").firstElementChild, null);
    assert_equals(document.querySelector("#placeholder").firstElementChild.textContent, "window.state = 'patched';");
    assert_equals(window.state, "patched");
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
        "byte_end": 407,
        "byte_start": 397,
        "col": 1,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.section.lacks_heading",
      "message": "Section lacks heading. Consider using “h2”-“h6” elements to add identifying headings to all sections, or else use a “div” element instead for any cases where no heading is needed.",
      "severity": "Warning",
      "span": {
        "byte_end": 526,
        "byte_start": 516,
        "col": 109,
        "line": 13
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
  "source_name": "html/dom/partial-updates/tentative/template-contentmethod-script-children.html"
}
```
