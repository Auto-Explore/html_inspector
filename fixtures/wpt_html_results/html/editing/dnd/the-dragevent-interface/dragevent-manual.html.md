# html/editing/dnd/the-dragevent-interface/dragevent-manual.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/the-dragevent-interface/dragevent-manual.html",
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
    <title>HTML Test: DragEvent</title>
    <link rel='author' title='Intel' href='http://www.intel.com'>
    <link rel='help' href='https://html.spec.whatwg.org/multipage/#dndevents'>
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
      var drag, element;
      var Events = ['ondragstart', 'ondrag', 'ondragover', 'ondragenter', 'ondragleave', 'ondrop', 'ondragend'];

      setup(function() {
          drag = document.querySelector('#drag');
          element = document.createElement('div');
        }, {explicit_done: true, explicit_timeout: true});

      for(var i=0; i< Events.length; i++) {
        test(function() {
          assert_true(Events[i] in document, 'Check ' + Events[i] + ' in document');
        }, 'Check ' + Events[i] + ' in document');
      }

      test(function() {
        assert_inherits(element, 'ondragstart', 'Check if have ondragstart attribute');
      }, 'Check if have ondragstart attribute');

      test(function() {
        assert_inherits(element, 'ondrag', 'Check if have ondrag attribute');
      }, 'Check if have ondrag attribute');

      test(function() {
        assert_inherits(element, 'ondragenter', 'Check if have ondragenter attribute');
      }, 'Check if have ondragenter attribute');

      test(function() {
        assert_inherits(element, 'ondragleave', 'Check if have dragleave attribute');
      }, 'Check if have dragleave attribute');

      test(function() {
        assert_inherits(element, 'ondragover', 'Check if have dragover attribute');
      }, 'Check if have dragover attribute');

      test(function() {
        assert_inherits(element, 'ondrop', 'Check if have ondrop attribute');
      }, 'Check if have ondrop attribute');

      test(function() {
        assert_inherits(element, 'ondragend', 'Check if have ondragend attribute');
      }, 'Check if have ondragend attribute');

      on_event(drag, 'dragstart', function(event) {
        test(function() {
          assert_equals(event.type, 'dragstart', 'Check if the dragstart event captured');
        }, 'Check if the dragstart event captured');
      });

      on_event(drag, 'dragenter', function(event) {
        test(function() {
          assert_equals(event.type, 'dragenter', 'Check if the dragenter event captured');
        }, 'Check if the dragenter event captured');
      });

      on_event(drag, 'dragend', function(event) {
        test(function() {
          assert_equals(event.type, 'dragend', 'Check if the dragend event captured');
        }, 'Check if the dragend event captured');
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
  "source_name": "html/editing/dnd/the-dragevent-interface/dragevent-manual.html"
}
```
