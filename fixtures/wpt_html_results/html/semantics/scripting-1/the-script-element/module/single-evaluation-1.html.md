# html/semantics/scripting-1/the-script-element/module/single-evaluation-1.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/module/single-evaluation-1.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Single evaluation, 1</title>

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
    window.log = [];

    const test_load = async_test(
        "Test that a module is evaluated only once, and that 'this' is " +
        "undefined (because of strict mode).");
    window.addEventListener("load", test_load.step_func_done(ev => {
      assert_array_equals(log, [undefined, "this-nested"]);
    }));
</script>
<script type="module" src="this.js"></script>
<script type="module" src="this.js"></script>
<script type="module" src="this-nested.js"></script>
<script type="module" src="this.js"></script>
<script type="module" src="this-nested.js"></script>
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
  "source_name": "html/semantics/scripting-1/the-script-element/module/single-evaluation-1.html"
}
```
