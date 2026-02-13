# html/semantics/scripting-1/the-script-element/module/dynamic-import/code-cache-base-url.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/module/dynamic-import/code-cache-base-url.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<!--
Regression test for https://crbug.com/990810:
Functions with the same source code shouldn't be cached if their referencing
scripts and host defined options are different.

Each Function in the following three scripts should have different base URLs
respectively, but in https://crbug.com/990810 the Function in
`gamma/code-cache.js` reuses the Function in `beta/code-cache.js`, resulting in
wrong base URL.
-->

<script src="alpha/code-cache.js"></script>
<script src="beta/code-cache.js"></script>
<script src="gamma/code-cache.js"></script>
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
  "source_name": "html/semantics/scripting-1/the-script-element/module/dynamic-import/code-cache-base-url.html"
}
```
