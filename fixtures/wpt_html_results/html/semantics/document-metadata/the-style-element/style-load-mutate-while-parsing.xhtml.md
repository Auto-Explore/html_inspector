# html/semantics/document-metadata/the-style-element/style-load-mutate-while-parsing.xhtml

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-style-element/style-load-mutate-while-parsing.xhtml",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<html xmlns="http://www.w3.org/1999/xhtml">
 <head>
  <title>The 'load' event on the style element should not fire for mutations while it's on the stack of open elements</title>
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
  <style>
   body { background-color: transparent; }

   <script>
   window.t = async_test((t) => {
     window.style = document.querySelector('style');
     style.onload = t.unreached_func('load event fired for mutation');
     style.append('body { color: inherit; }');
   });
   </script>

   <script>
   style.onload = t.step_func(() => {
     t.done();
   });
   </script>
  </style>
 </head>
 <body/>
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
  "source_name": "html/semantics/document-metadata/the-style-element/style-load-mutate-while-parsing.xhtml"
}
```
