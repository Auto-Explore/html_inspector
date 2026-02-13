# html/semantics/scripting-1/the-script-element/module/inline-async-execorder.html

Counts:
- errors: 1
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/module/inline-async-execorder.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<html>
  <head>
    <title>Inline async module script execution order</title>
    <meta name=timeout content=long>
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
  </head>
  <body>
    <script>
      let loaded = [];
      let test = async_test("Inline async module script execution order");
      window.addEventListener("load", test.step_func(function() {
        assert_array_equals(loaded,
                            ["fast", "fast", "fast", "slow", "slow", "slow"]);
      test.done();
      }));
    </script>
    <script type="module" async src="resources/slow-module.js?pipe=trickle(d2)&unique=1"></script>
    <script type="module" async>
      import "./resources/slow-module.js?pipe=trickle(d2)&unique=2";
      loaded.push("slow");
    </script>
    <script type="module" async src="resources/fast-module.js?unique=1"></script>
    <script type="module" async>
      import "./resources/fast-module.js?unique=2";
      loaded.push("fast");
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
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/scripting-1/the-script-element/module/inline-async-execorder.html"
}
```
