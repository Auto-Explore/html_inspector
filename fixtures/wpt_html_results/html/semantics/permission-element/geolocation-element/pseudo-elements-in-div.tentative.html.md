# html/semantics/permission-element/geolocation-element/pseudo-elements-in-div.tentative.html

Counts:
- errors: 1
- warnings: 5
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/permission-element/geolocation-element/pseudo-elements-in-div.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<link rel="match" href="pseudo-elements-in-div-ref.html">
<link rel="help" href="https://github.com/WICG/PEPC/blob/main/explainer.md#locking-the-pepc-style">
<link rel="help" href="https://crbug.com/342355738">
<body>
  <div>
    The geolocation element should not create any pseudo elements.
  </div>

  <style>
    /* The space (descendent combinator) is necessary to apply this CSS to children of .outer */
    .outer ::before {
      content: 'BEFORE';
    }
    .outer ::after {
      content: 'AFTER';
    }
  </style>

  <p>Should only see BEFOREAFTER once below</p>

  <!-- Apply the ::before/::after to descendants of the .outer div. The empty inner div
     simulates a quirk scenario discovered in chromium, but the expectation should match
     all implementation. -->
  <div class="outer">
    <div></div>
    <geolocation></geolocation>
  </div>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.style.not_allowed_here",
      "message": "Element “style” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 349,
        "byte_start": 342,
        "col": 3,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “geolocation” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 873,
        "byte_start": 860,
        "col": 5,
        "line": 28
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “geolocation” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 873,
        "byte_start": 860,
        "col": 5,
        "line": 28
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “html”.",
      "severity": "Error",
      "span": {
        "byte_end": 912,
        "byte_start": 905,
        "col": 1,
        "line": 31
      }
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
  "source_name": "html/semantics/permission-element/geolocation-element/pseudo-elements-in-div.tentative.html"
}
```
