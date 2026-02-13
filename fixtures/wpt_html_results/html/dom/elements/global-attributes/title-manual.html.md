# html/dom/elements/global-attributes/title-manual.html

Counts:
- errors: 0
- warnings: 5
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/elements/global-attributes/title-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>The title attribute</title>
<style>
div > * { display: inline }
link::before { content: "link" }
</style>
<p>Hover each word below. The tooltip for each of them should be "PASS".</p>
<div title=PASS>div <link> <style>style</style> <dfn>dfn</dfn> <abbr>abbr</abbr> <menuitem>menuitem</menuitem></div>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.link.href.required",
      "message": "A “link” element must have an “href” or “imagesrcset” attribute, or both.",
      "severity": "Warning",
      "span": {
        "byte_end": 232,
        "byte_start": 226,
        "col": 21,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.style.not_allowed_here",
      "message": "Element “style” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 240,
        "byte_start": 233,
        "col": 28,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 297,
        "byte_start": 287,
        "col": 82,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.element.menuitem.obsolete",
      "message": "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 297,
        "byte_start": 287,
        "col": 82,
        "line": 8
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
  "source_name": "html/dom/elements/global-attributes/title-manual.html"
}
```
