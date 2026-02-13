# html/editing/dnd/the-dropzone-attribute/dropzone_attribute_inputbox_element_dbcs-manual.html

Counts:
- errors: 1
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/the-dropzone-attribute/dropzone_attribute_inputbox_element_dbcs-manual.html",
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
    <title>HTML Test: dropzone_attribute_inputbox_element_dbcs</title>
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
      input {
        color: blue;
        margin: 20px auto;
      }
    </style>
  </head>

  <body>
    <div>Select all the inputbox DBCS (Double Byte Character Set) text then drag to rectangular box.</div>
    <input draggable='true' type='text' value='您好，互联网。'></input>
    <div id='drop' dropzone='move string:text/plain'></div>
    <div id='log'> </div>

    <script>
      var drop;
      setup(function() {
          drop = document.querySelector('#drop');
      }, {explicit_done: true, explicit_timeout: true});

      on_event(drop, 'drop', function(event) {
        test(function() {
          var item = item = event.dataTransfer.items[0];
          assert_equals(event.dataTransfer.getData(item.type), '您好，互联网。',  'The dropped string value is the inputbox text you selected.');
        }, 'The dropped string value is the inputbox text you selected.');

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
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “input”.",
      "severity": "Error",
      "span": {
        "byte_end": 825,
        "byte_start": 817,
        "col": 71,
        "line": 26
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
  "source_name": "html/editing/dnd/the-dropzone-attribute/dropzone_attribute_inputbox_element_dbcs-manual.html"
}
```
