# html/semantics/scripting-1/the-script-element/module/inline-defer-onload.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/module/inline-defer-onload.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<html>
  <head>
    <title>Inline defer module script without external deps onload blocking</title>
    <meta name=timeout content=long>
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
  </head>
  <body>
    <script>
      let loadFired = false;
      let moduleRan = false
      let test = async_test("Inline defer module script vs. onload");
      window.addEventListener("load", test.step_func(function() {
        loadFired = true;
        assert_true(moduleRan, "Module should have run before the load event");
        test.step_timeout(function() {
          test.done();
        }, 0);
      }));
    </script>
    <!-- defer should be equivalent to neither defer nor async specified -->
    <script type="module" defer>
      moduleRan = true;
      test.step_func(function() {
        assert_false(loadFired, "onload should not have fired yet");
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
      "code": "html.parser.start_tag_without_doctype",
      "message": "Start tag seen without seeing a doctype first. Expected “<!DOCTYPE html>”.",
      "severity": "Error",
      "span": {
        "byte_end": 6,
        "byte_start": 0,
        "col": 1,
        "line": 1
      }
    },
    {
      "category": "Html",
      "code": "html.script.module.defer.disallowed",
      "message": "A “script” element with “type=module” must not have a “defer” attribute.",
      "severity": "Warning",
      "span": {
        "byte_end": 793,
        "byte_start": 765,
        "col": 5,
        "line": 22
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
  "source_name": "html/semantics/scripting-1/the-script-element/module/inline-defer-onload.html"
}
```
