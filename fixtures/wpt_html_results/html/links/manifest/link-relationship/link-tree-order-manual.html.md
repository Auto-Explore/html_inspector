# html/links/manifest/link-relationship/link-tree-order-manual.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/links/manifest/link-relationship/link-tree-order-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Test that name member is supported</title>
<link rel="help" href="https://html.spec.whatwg.org/#link-type-manifest" />
<link rel="manifesto" href="/appmanifest/name-member/name-member-fail.webmanifest" />
<link
  rel="hello manifest another-relationship"
  href="/appmanifest/name-member/name-member.webmanifest"
/>
<link rel="manifest" href="/appmanifest/name-member/name-member-fail.webmanifest" />
<link rel="hello manifest" href="/appmanifest/name-member/name-member-fail.webmanifest" />
<p>
  If when installing the name is "pass" then the test has passed.
</p>
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
  "source_name": "html/links/manifest/link-relationship/link-tree-order-manual.html"
}
```
