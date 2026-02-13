# html/editing/dnd/the-datatransfer-interface/dndTransferCases-manual.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/the-datatransfer-interface/dndTransferCases-manual.html",
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
    <title>HTML Test: dropzone_attribute_data_item_kind_string</title>
    <link rel='author' title='Intel' href='http://www.intel.com'>
    <link rel='author' title='Domenic Denicola' href='mailto:d@domenic.com'>
    <link rel='help' href='https://html.spec.whatwg.org/multipage/#the-datatransfer-interface'>
    <script src='/resources/testharness.js'></script>
    <script src='/resources/testharnessreport.js'></script>
    <style>
      #drop {
        border: 2px solid black;
        width: 100px;
        height: 100px;
        padding: 20px;
      }
      #drag {
        color: blue;
        margin: 20px auto;
      }
    </style>
  </head>

  <body>
    <div>Select and drag the blue text to rectangular box.</div>
    <div id='drag' draggable>blue text</div>
    <div id='drop' dropzone='copy string:text/plain'></div>
    <div id='log'> </div>

    <script>
      var drag;
      setup(function() {
          drag = document.querySelector('#drag');
      }, {explicit_done: true, explicit_timeout: true});

      on_event(drag, 'dragstart', function(event) {
        test(function() {
          assert_equals(event.dataTransfer.effectAllowed, 'uninitialized');
        }, 'effectAllowed should be "uninitialized"');

        test(function() {
          assert_equals(event.dataTransfer.types.constructor, Array, 'should be an array');
          assert_true(Object.isFrozen(event.dataTransfer.types), 'should be frozen');
        }, 'types should be a frozen array');

        test(function() {
          assert_false('contains' in event.dataTransfer.types);
          assert_false('item' in event.dataTransfer.types);
        }, 'types should not have any of the historical methods');

        test(function() {
          assert_equals(event.dataTransfer.types, event.dataTransfer.types);
        }, 'types should return the same object from multiple reads (assuming no changes)');

        test(function() {
          var before = event.dataTransfer.types;
          event.dataTransfer.clearData();
          assert_not_equals(event.dataTransfer.types, before);
        }, 'types should return a different object after changes');

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
  "source_name": "html/editing/dnd/the-datatransfer-interface/dndTransferCases-manual.html"
}
```
