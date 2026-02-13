# html/semantics/forms/the-input-element/url.html

Counts:
- errors: 0
- warnings: 6
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/url.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
  <title>Input url</title>
  <link rel="author" title="Hyeonseok Shin" href="mailto:hyeonseok@gmail.com">
  <link rel="help" href="https://html.spec.whatwg.org/multipage/#url-state-%28type=url%29">
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
</head>
<body>
  <h1>Input url</h1>
  <div style="display: none">
  <input type="url" id="type_support" />
  <input type="url" id="set_value_LF" />
  <input type="url" id="set_value_CR" />
  <input type="url" id="set_value_CRLF" />
  <input type="url" id="value_with_CRLF" value="a&#x000D;&#x000A;a" />
  <input type="url" id="value_with_leading_trailing_white_space" value=" aa " />
  <input type="url" id="value_with_leading_trailing_inner_white_space" value=" a a " />
  </div>
  <div id="log">
  </div>

  <script type="text/javascript">
    test(function(){
      var element = document.getElementById('type_support');
      assert_equals(element.type, 'url');
    }, 'url type supported on input element');

    test(function(){
      var element = document.getElementById('set_value_LF');
      element.value = 'a\u000Aa';
      assert_equals(element.value, 'aa');

      element = document.getElementById('set_value_CR');
      element.value = 'a\u000Da';
      assert_equals(element.value, 'aa');

      element = document.getElementById('set_value_CRLF');
      element.value = 'a\u000D\u000Aa';
      assert_equals(element.value, 'aa');
    }, 'The value must not be set with "LF" (U+000A) or "CR" (U+000D)');

    test(function(){
      var element = document.getElementById('value_with_CRLF');
      assert_equals(element.value, 'aa');
    }, 'The value sanitization algorithm is as follows: Strip line breaks from the value');

    test(function(){
      var element = document.getElementById('value_with_leading_trailing_white_space');
      assert_equals(element.value, 'aa');

      element = document.getElementById('value_with_leading_trailing_inner_white_space');
      assert_equals(element.value, 'a a');
    }, 'The value sanitization algorithm is as follows: Strip leading and trailing whitespace from the value.');
  </script>

</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.input.url.value.invalid",
      "message": "Bad value “a\r\na” for attribute “value” on element “input”.",
      "severity": "Warning",
      "span": {
        "byte_end": 640,
        "byte_start": 572,
        "col": 3,
        "line": 17
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_cr",
      "message": "A numeric character reference expanded to carriage return.",
      "severity": "Warning",
      "span": {
        "byte_end": 574,
        "byte_start": 573,
        "col": 3,
        "line": 17
      }
    },
    {
      "category": "Html",
      "code": "html.input.url.value.invalid",
      "message": "Bad value “aa” for attribute “value” on element “input”.",
      "severity": "Warning",
      "span": {
        "byte_end": 721,
        "byte_start": 643,
        "col": 3,
        "line": 18
      }
    },
    {
      "category": "Html",
      "code": "html.input.url.value.invalid",
      "message": "Bad value “a a” for attribute “value” on element “input”.",
      "severity": "Warning",
      "span": {
        "byte_end": 809,
        "byte_start": 724,
        "col": 3,
        "line": 19
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 879,
        "byte_start": 848,
        "col": 3,
        "line": 24
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
  "source_name": "html/semantics/forms/the-input-element/url.html"
}
```
