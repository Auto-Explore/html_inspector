# html/semantics/interactive-elements/the-details-element/details-cq-crash.html

Counts:
- errors: 0
- warnings: 6
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-details-element/details-cq-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:vmpstr@chromium.org">
<link rel=help href="https://bugs.chromium.org/p/chromium/issues/detail?id=1334983">

<canvas>
  <details>
    <card card>

<script>
async function trigger() {
 document.querySelector("canvas").style.setProperty("container-type", "size");
 document.querySelector("canvas").style.setProperty("column-span", "all");
 document.querySelector("card").setAttribute("contenteditable", "true");
}
onload = requestAnimationFrame(() => requestAnimationFrame(trigger));
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “card” not allowed as child of “details” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 190,
        "byte_start": 179,
        "col": 5,
        "line": 7
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “card” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 190,
        "byte_start": 179,
        "col": 5,
        "line": 7
      }
    },
    {
      "category": "Html",
      "code": "html.mathml.element.outside_math",
      "message": "Element “card” not allowed as child of “details” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 190,
        "byte_start": 179,
        "col": 5,
        "line": 7
      }
    },
    {
      "category": "Html",
      "code": "html.details.missing_summary",
      "message": "Element “details” is missing a required instance of child element “summary”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
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
  "source_name": "html/semantics/interactive-elements/the-details-element/details-cq-crash.html"
}
```
