# html/rendering/non-replaced-elements/the-page/body_link.xhtml

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-page/body_link.xhtml",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<html xmlns='http://www.w3.org/1999/xhtml'>
<head>
<title>body - LINK=yellow</title>
</head>
<body link="yellow">
<p> Test for <b> link="yellow" </b> on body </p>

This <a href="test-body.xhtml">LINK</a> should be displayed in <b>yellow</b><i> if it has not been clicked before </i><br/>
<p>Once clicked, the link will take default color of visited link.<br /></p>
<p>To run this test again in browsers, delete your browsing history and navigate to this page.<br /></p>

<p>
<i>Note - This test checks for User Agent requirement as per HTML5 spec NOT the author requirement</i>
</p>
</body>
</html>
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
  "source_name": "html/rendering/non-replaced-elements/the-page/body_link.xhtml"
}
```
