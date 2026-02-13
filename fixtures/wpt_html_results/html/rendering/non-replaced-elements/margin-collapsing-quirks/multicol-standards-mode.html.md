# html/rendering/non-replaced-elements/margin-collapsing-quirks/multicol-standards-mode.html

Counts:
- errors: 0
- warnings: 5
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/margin-collapsing-quirks/multicol-standards-mode.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>multicol default styles (standards mode)</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/#multicol">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<multicol id=test></multicol>
<asdfasdf id=ref></asdfasdf>
<script src="compare-computed-style.js"></script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “multicol” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 270,
        "byte_start": 252,
        "col": 1,
        "line": 6
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “multicol” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 270,
        "byte_start": 252,
        "col": 1,
        "line": 6
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “asdfasdf” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 299,
        "byte_start": 282,
        "col": 1,
        "line": 7
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “asdfasdf” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 299,
        "byte_start": 282,
        "col": 1,
        "line": 7
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
  "source_name": "html/rendering/non-replaced-elements/margin-collapsing-quirks/multicol-standards-mode.html"
}
```
