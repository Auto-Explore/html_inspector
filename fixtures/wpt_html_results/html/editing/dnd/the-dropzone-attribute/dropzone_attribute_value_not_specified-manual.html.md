# html/editing/dnd/the-dropzone-attribute/dropzone_attribute_value_not_specified-manual.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/the-dropzone-attribute/dropzone_attribute_value_not_specified-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
  <head>
    <meta charset='utf-8'>
    <title>HTML Test: dropzone_attribute_value_not_specified</title>
    <link rel='author' title='Intel' href='http://www.intel.com'>
    <link rel='help' href='https://html.spec.whatwg.org/multipage/#the-dropzone-attribute'>
    <script src='/resources/testharness.js'></script>
    <script src='/resources/testharnessreport.js'></script>
    <style>
      #drop {
        border: 2px solid black;
        width: 100px;
        height: 100px;
        padding: 20px;
      }
      #select {
        color: blue;
        margin: 20px auto;
      }
    </style>
  </head>

  <body>
    <div>Select and drag the blue text to rectangular box.</div>
    <div id='select' draggable>blue text</div>
    <div id='drop' dropzone='string:text/plain'></div>
    <div id='log'> </div>

    <script>
      var drop;
      setup(function() {
          drop = document.querySelector('#drop');
      }, {explicit_done: true, explicit_timeout: true});

      on_event(drop, 'drop', function(event) {
        test(function() {
          assert_equals(event.dataTransfer.dropEffect, 'copy', 'dropzone content attribute value is "copy" if it\'s not specified');
        }, 'dropzone content attribute value is "copy" if it\'s not specified');

        done();
      });
    </script>
  </body>
</html>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/editing/dnd/the-dropzone-attribute/dropzone_attribute_value_not_specified-manual.html"
}
```
