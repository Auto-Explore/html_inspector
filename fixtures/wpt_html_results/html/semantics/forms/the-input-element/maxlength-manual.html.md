# html/semantics/forms/the-input-element/maxlength-manual.html

Counts:
- errors: 1
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/maxlength-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
  <head>
    <meta charset=utf-8>
    <title>input max length</title>
    <link rel="author" title="Sam Gibson" href="mailto:sam@ifdown.net">
    <link rel=help href="https://html.spec.whatwg.org/multipage/forms.html#the-maxlength-and-minlength-attributes">
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
  </head>

  <body>
    <div id="log"></div>
    <p>Type a letter anywhere into the input field (do not select any text, or otherwise manipulate the input)</p>
    <input type=text maxlength=4 id=only-four value="inpu"></input>

    <script>
      var input;
      setup(function() {
        input = document.getElementById('only-four');
      }, {explicit_done: true, explicit_timeout: true});


      on_event(input, 'keyup', function(event) {
          if  ((event.keyCode >= 65 && event.keyCode <= 90) ||
                (event.keyCode >= 97 && event.keyCode <= 122)) {
            test(function() {
              assert_equals(input.value, "inpu");
            }, 'input content should limit to maxlength')

            done();
          }
      });
    </script>
  </body>
</html>

```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “input”.",
      "severity": "Error",
      "span": {
        "byte_end": 622,
        "byte_start": 614,
        "col": 60,
        "line": 15
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
  "source_name": "html/semantics/forms/the-input-element/maxlength-manual.html"
}
```
