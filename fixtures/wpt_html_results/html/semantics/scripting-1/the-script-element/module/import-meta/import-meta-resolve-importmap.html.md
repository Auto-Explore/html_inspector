# html/semantics/scripting-1/the-script-element/module/import-meta/import-meta-resolve-importmap.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/module/import-meta/import-meta-resolve-importmap.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<!--
  More extensive tests of import maps and import.meta.resolve() will be
  located in the import maps test suite. This contains some basic tests plus
  tests some tricky parts of the import.meta.resolve() algorithm around string
  conversion which are only testable with import maps.
-->

<script type="importmap">
{
  "imports": {
    "bare": "https://example.com/",
    "https://example.com/rewrite": "https://example.com/rewritten",

    "1": "https://example.com/PASS-1",
    "null": "https://example.com/PASS-null",
    "undefined": "https://example.com/PASS-undefined",
    "[object Object]": "https://example.com/PASS-object",

    "./start": "./resources/export-1.mjs",
    "./resources/export-1.mjs": "./resources/export-2.mjs"
  }
}
</script>

<script type="module">
test(() => {
  assert_equals(import.meta.resolve("bare"), "https://example.com/");
}, "import.meta.resolve() given an import mapped bare specifier");

test(() => {
  assert_equals(import.meta.resolve("https://example.com/rewrite"), "https://example.com/rewritten");
}, "import.meta.resolve() given an import mapped  URL-like specifier");

test(() => {
  assert_equals(import.meta.resolve(), "https://example.com/PASS-undefined", "no-arg case");

  assert_equals(import.meta.resolve(1), "https://example.com/PASS-1");
  assert_equals(import.meta.resolve(null), "https://example.com/PASS-null");
  assert_equals(import.meta.resolve(undefined), "https://example.com/PASS-undefined");

  // Only toString() methods are consulted by ToString, not valueOf() ones.
  // So this becomes "[object Object]".
  assert_equals(import.meta.resolve({ valueOf() { return "./x"; } }), "https://example.com/PASS-object");
}, "Testing the ToString() step of import.meta.resolve() via import maps");

promise_test(async () => {
  const one = (await import("./start")).default;
  assert_equals(one, 1);

  const two = (await import(import.meta.resolve("./start"))).default;
  assert_equals(two, 2);
}, "import(import.meta.resolve(x)) can be different from import(x)");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/scripting-1/the-script-element/module/import-meta/import-meta-resolve-importmap.html"
}
```
