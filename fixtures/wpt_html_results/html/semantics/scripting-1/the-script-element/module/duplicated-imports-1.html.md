# html/semantics/scripting-1/the-script-element/module/duplicated-imports-1.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/module/duplicated-imports-1.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Importing a module multiple times with the same specifier</title>

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
window.log = [];
</script>
<script type="module">
import { foo } from './export-something.js';
import { set_foo } from './export-something.js';
import default1 from './export-default.js';
import default2 from './export-default.js';

test(() => {
  assert_array_equals(log, ['export-something', 'export-default']);
  assert_equals(foo, 42);
  set_foo(43);
  assert_equals(foo, 43);
  assert_equals(default1, "fox");
  assert_equals(default2, "fox");
}, 'Duplicated imports with the same specifier');
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
  "source_name": "html/semantics/scripting-1/the-script-element/module/duplicated-imports-1.html"
}
```
