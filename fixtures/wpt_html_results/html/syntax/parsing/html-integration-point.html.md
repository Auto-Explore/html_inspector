# html/syntax/parsing/html-integration-point.html

Counts:
- errors: 0
- warnings: 5
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/parsing/html-integration-point.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="help" href="https://html.spec.whatwg.org/multipage/parsing.html#tree-construction:html-integration-point">
<body>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<math><annotation-xml id="point-1" encoding="text/html"><xmp>&lt;/xmp&gt;&lt;img></xmp></annotation-xml></math>
<math><annotation-xml id="point-2" encoding="application/xhtml+xml"><style>&lt;/style&gt;&lt;img></style></annotation-xml></math>
<svg><foreignObject id="point-3"><iframe>&lt;/iframe&gt;&lt;img></iframe></foreignObject></svg>
<svg><desc id="point-4"><noembed>&lt;/noembed&gt;&lt;img></noembed></desc></svg>
<svg><title id="point-5"><noframes>&lt;/noframes&gt;&lt;img></noframes></title></svg>

<script>
function generate_test(id) {
  return () => {
    let point = document.querySelector('#' + id);
    assert_not_equals(point.namespaceURI, 'http://www.w3.org/1999/xhtml');
    let rawTextElement = point.firstChild;
    assert_equals(rawTextElement.namespaceURI, 'http://www.w3.org/1999/xhtml');
    assert_equals(rawTextElement.textContent.substr(0, 4), '&lt;',
                  'Entity references should not be decoded.');
  };
}

test(generate_test('point-1'), 'MathML annotation-xml with encoding=text/html should be an HTML integration point');
test(generate_test('point-2'), 'MathML annotation-xml with encoding=application/xhtml+xml should be an HTML integration point');
test(generate_test('point-3'), 'SVG foreignObject should be an HTML integration point');
test(generate_test('point-4'), 'SVG desc should be an HTML integration point');
test(generate_test('point-5'), 'SVG title should be an HTML integration point');
</script>
</body>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.style.not_allowed_here",
      "message": "Element “style” not allowed as child of “annotation-xml” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 435,
        "byte_start": 428,
        "col": 69,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.iframe.text.disallowed",
      "message": "Text not allowed in “iframe” in this context.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "Html",
      "code": "html.element.noframes.obsolete",
      "message": "The “noframes” element is obsolete. Use the “iframe” element and CSS instead, or use server-side includes.",
      "severity": "Warning",
      "span": {
        "byte_end": 702,
        "byte_start": 692,
        "col": 26,
        "line": 11
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
  "source_name": "html/syntax/parsing/html-integration-point.html"
}
```
