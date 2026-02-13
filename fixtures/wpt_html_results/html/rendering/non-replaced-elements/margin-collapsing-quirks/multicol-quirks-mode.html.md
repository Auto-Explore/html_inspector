# html/rendering/non-replaced-elements/margin-collapsing-quirks/multicol-quirks-mode.html

Counts:
- errors: 1
- warnings: 5
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/margin-collapsing-quirks/multicol-quirks-mode.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<title>multicol default styles (quirks mode)</title>
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
      "code": "html.parser.start_tag_without_doctype",
      "message": "Start tag seen without seeing a doctype first. Expected “<!DOCTYPE html>”.",
      "severity": "Error",
      "span": {
        "byte_end": 7,
        "byte_start": 0,
        "col": 1,
        "line": 1
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “multicol” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 251,
        "byte_start": 233,
        "col": 1,
        "line": 5
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “multicol” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 251,
        "byte_start": 233,
        "col": 1,
        "line": 5
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “asdfasdf” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 280,
        "byte_start": 263,
        "col": 1,
        "line": 6
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “asdfasdf” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 280,
        "byte_start": 263,
        "col": 1,
        "line": 6
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
  "source_name": "html/rendering/non-replaced-elements/margin-collapsing-quirks/multicol-quirks-mode.html"
}
```
