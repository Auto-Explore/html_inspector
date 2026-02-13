# html/editing/dnd/the-dropzone-attribute/dropzone_attribute.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/the-dropzone-attribute/dropzone_attribute.html",
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
    <title>HTML Test: dropzone_attribute</title>
    <link rel='author' title='Intel' href='http://www.intel.com'>
    <link rel='help' href='https://html.spec.whatwg.org/multipage/#the-dropzone-attribute'>
    <meta name="flags" content="may">
    <script src='/resources/testharness.js'></script>
    <script src='/resources/testharnessreport.js'></script>
  </head>

  <body>
    <div id='log'> </div>

    <script>
        var drop_element;

        setup(function() { drop_element = document.createElement('div'); });

        test(function() {
          //Empty values for elements
          drop_element.dropzone = '';
          assert_not_equals(drop_element.dropzone, undefined, 'div.dropzone should not be undefined if it\'s been set');
        }, 'div.dropzone should not be undefined if it\'s been set');

        test(function() {
          drop_element.dropzone = null;
          assert_not_equals(drop_element.dropzone, null,  'div.dropzone should not be null');
        }, 'div.dropzone should not be null');

        test(function() {
          //The dropzone IDL attribute must reflect the content attribute of the same name.
          drop_element.setAttribute('dropzone', 'copy file:image/png file:image/gif file:image/jpeg');
          assert_equals(drop_element.dropzone, 'copy file:image/png file:image/gif file:image/jpeg', 'div dropzone idl attribute must reflect the content attribute of the same name');
        }, 'div dropzone idl attribute must reflect the content attribute of the same name');

         test(function() {
          //The dropzone content attribute is set to the literal value when the idl attribute value is set.
          drop_element.dropzone = 'copy file:image/png file:image/gif file:image/jpeg';
          assert_equals(drop_element.getAttribute('dropzone'), 'copy file:image/png file:image/gif file:image/jpeg', 'div dropzone content attribute is set to the literal value');
        }, 'div dropzone content attribute is set to the literal value');
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
  "source_name": "html/editing/dnd/the-dropzone-attribute/dropzone_attribute.html"
}
```
