# html/semantics/scripting-1/the-script-element/module/late-star-export-request.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/module/late-star-export-request.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Late star-export request</title>

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
    window.log = [];

    const test_load = async_test(
        "Test the situation where a module is instantiated without a use of " +
        "its star-exports, but later on a different module requests them.");
    window.addEventListener("load", test_load.step_func_done(ev => {
      assert_array_equals(log, [
          "export-something", "export-something-nested",
          "import-something-namespace", 42, 43]);
    }));
</script>
<script type="module" src="export-something-nested.js"></script>
<script type="module">
    log.push("import-something-namespace");
    log.push(foo);
    set_foo(43);
    log.push(foo);
    import {foo, set_foo} from "./export-something-nested.js";
</script>
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
  "source_name": "html/semantics/scripting-1/the-script-element/module/late-star-export-request.html"
}
```
