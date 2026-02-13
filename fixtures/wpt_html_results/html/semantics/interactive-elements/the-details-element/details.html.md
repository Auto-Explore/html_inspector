# html/semantics/interactive-elements/the-details-element/details.html

Counts:
- errors: 0
- warnings: 5
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-details-element/details.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title>HTML details element API</title>
    <style>#one, #two { visibility: hidden; }</style>
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
  </head>
  <body>
    <div id="log"></div>

    <!-- Used in parsing tests -->
    <div id='one'><details></details><details></details></div>
    <div id='two'><p><details></details></div>

    <script type="text/javascript">

function makeDetails () {
  return document.createElement('details');
}


// <details>
test(function () {
  var times = document.getElementById('one').getElementsByTagName('details');
  assert_equals( times.length, 2 );
}, 'HTML parsing should locate 2 details elements in this document');

test(function () {
  assert_equals( document.getElementById('two').getElementsByTagName('p')[0].innerHTML, '' );
}, 'HTML parsing should close an unclosed <p> before <details>');

test(function () {
  assert_true( !!window.HTMLDetailsElement );
}, 'HTMLDetailsElement should be exposed for prototyping');

test(function () {
  assert_true( makeDetails() instanceof window.HTMLDetailsElement);
}, 'a dynamically created details element should be instanceof HTMLDetailsElement');

test(function () {
  assert_true( document.getElementById('one').getElementsByTagName('details')[0] instanceof window.HTMLDetailsElement);
}, 'a details element from the parser should be instanceof HTMLDetailsElement');
    </script>

  </body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.details.missing_summary",
      "message": "Element “details” is missing a required instance of child element “summary”.",
      "severity": "Warning",
      "span": {
        "byte_end": 361,
        "byte_start": 351,
        "col": 28,
        "line": 13
      }
    },
    {
      "category": "Html",
      "code": "html.details.missing_summary",
      "message": "Element “details” is missing a required instance of child element “summary”.",
      "severity": "Warning",
      "span": {
        "byte_end": 380,
        "byte_start": 370,
        "col": 47,
        "line": 13
      }
    },
    {
      "category": "Html",
      "code": "html.details.missing_summary",
      "message": "Element “details” is missing a required instance of child element “summary”.",
      "severity": "Warning",
      "span": {
        "byte_end": 427,
        "byte_start": 417,
        "col": 31,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 470,
        "byte_start": 439,
        "col": 5,
        "line": 16
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
  "source_name": "html/semantics/interactive-elements/the-details-element/details.html"
}
```
