# html/editing/dnd/the-dropzone-attribute/dropzone_attribute_data_item_kind_file-manual.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/the-dropzone-attribute/dropzone_attribute_data_item_kind_file-manual.html",
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
    <title>HTML Test: dropzone_attribute_data_item_kind_file</title>
    <link rel='author' title='Intel' href='http://www.intel.com'>
    <link rel='help' href='https://html.spec.whatwg.org/multipage/#the-dropzone-attribute'>
    <script src='/resources/testharness.js'></script>
    <script src='/resources/testharnessreport.js'></script>
    <style>
      #drop {
        border: 2px solid black;
        width: 160px;
        height: 160px;
        padding: 20px;
        color: white;
      }
      img {
        margin: 20px auto;
      }
    </style>
  </head>

  <body>
    <img src='/images/blue.png' alt='blue image' />
    <div>Save the blue image (image/png) above to your desktop, drag the image from desktop to the blue text to rectangular box in browser.</div>
    <div id='drop' dropzone='copy file:image/png'></div>
    <div id='log'> </div>

    <script>
      var drop;
      setup(function() {
          drop = document.querySelector('#drop');
      }, {explicit_done: true, explicit_timeout: true});

      on_event(drop, 'drop', function(event) {
        var item = event.dataTransfer.items[0];
        test(function() {
          assert_equals(item.kind, 'file',  'data item kind is file');
        }, 'data item kind is file');
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
  "source_name": "html/editing/dnd/the-dropzone-attribute/dropzone_attribute_data_item_kind_file-manual.html"
}
```
