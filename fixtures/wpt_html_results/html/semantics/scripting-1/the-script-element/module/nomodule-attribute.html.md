# html/semantics/scripting-1/the-script-element/module/nomodule-attribute.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/module/nomodule-attribute.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>The 'nomodule' attribute</title>

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
    window.log = [];

    const test_load = async_test(
        "Test that 'nomodule' has the desired effect on classic scripts, but " +
        "no effect on module scripts.");
    window.addEventListener("load", test_load.step_func_done(ev => {
      assert_array_equals(log, [undefined]);
    }));

</script>
<script type="module" src="this.js" nomodule></script>
<script src="this.js" nomodule></script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.script.module.nomodule.disallowed",
      "message": "A “script” element with a “nomodule” attribute must not have a “type” attribute with the value “module”.",
      "severity": "Warning",
      "span": {
        "byte_end": 529,
        "byte_start": 484,
        "col": 1,
        "line": 17
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
  "source_name": "html/semantics/scripting-1/the-script-element/module/nomodule-attribute.html"
}
```
