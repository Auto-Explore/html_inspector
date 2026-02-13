# html/semantics/scripting-1/the-script-element/module/import-meta/import-meta-resolve-multiple-scripts.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/module/import-meta/import-meta-resolve-multiple-scripts.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<iframe src="resources/store-import-meta.html"></iframe>

<script type="module">
import * as otherImportMeta from "./resources/export-import-meta.mjs";
setup({ explicit_done: true });

window.onload = () => {
  test(() => {
    assert_not_equals(frames[0].importMetaURL, import.meta.url,
      "Precondition check: we've set things up so that the other script has a different import.meta.url");

    const expected = (new URL("resources/x", location.href)).href;
    assert_equals(frames[0].importMetaResolve("./x"), expected);
  }, "import.meta.resolve resolves URLs relative to the import.meta.url, not relative to the active script when it is called: another global's inline script");

  test(() => {
    const otherFrameImportMetaResolve = frames[0].importMetaResolve;

    document.querySelector("iframe").remove();

    const expected = (new URL("resources/x", location.href)).href;
    assert_equals(otherFrameImportMetaResolve("./x"), expected);
  }, "import.meta.resolve still works if its global has been destroyed (by detaching the iframe)");

  test(() => {
    assert_not_equals(otherImportMeta.url, import.meta.url,
      "Precondition check: we've set things up so that the other script has a different import.meta.url");

    const expected = (new URL("resources/x", location.href)).href;
    assert_equals(otherImportMeta.resolve("./x"), expected);
  }, "import.meta.resolve resolves URLs relative to the import.meta.url, not relative to the active script when it is called: another module script");

  done();
};
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
  "source_name": "html/semantics/scripting-1/the-script-element/module/import-meta/import-meta-resolve-multiple-scripts.html"
}
```
