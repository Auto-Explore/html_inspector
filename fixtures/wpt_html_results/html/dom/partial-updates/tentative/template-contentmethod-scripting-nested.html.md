# html/dom/partial-updates/tentative/template-contentmethod-scripting-nested.html

Counts:
- errors: 1
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/partial-updates/tentative/template-contentmethod-scripting-nested.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<meta charset="utf-8" />
<title>HTML partial updates - a script inside a template inside a patch should not execute</title>
<link rel=help href="https://github.com/WICG/declarative-partial-updates">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<section id="placeholder" contentname="c">
    <script>
    window.state = "init";
    </script>
</section>
<template contentmethod="replace-children"><section contentname="c"><template><script>window.state = 'patched';</script></section></template></template>
<script>
test(() => {
    assert_not_equals(document.querySelector("#placeholder").firstElementChild, null);
    assert_equals(window.state, "init");
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
        "byte_end": 429,
        "byte_start": 419,
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
        "byte_end": 560,
        "byte_start": 550,
        "col": 121,
        "line": 13
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “template”.",
      "severity": "Error",
      "span": {
        "byte_end": 582,
        "byte_start": 571,
        "col": 142,
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
  "source_name": "html/dom/partial-updates/tentative/template-contentmethod-scripting-nested.html"
}
```
