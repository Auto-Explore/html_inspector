# html/rendering/non-replaced-elements/form-controls/toggle-display-ref.html

Counts:
- errors: 0
- warnings: 5
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/form-controls/toggle-display-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
<title>Reference: toggle 'display' test</title>

<div id="tests">
 <input type="hidden">
 <input type="text">
 <input type="text" value="a">
 <input type="search">
 <input type="search" value="a">
 <input type="tel">
 <input type="tel" value="123456789">
 <input type="url">
 <input type="url" value="http://a">
 <input type="email">
 <input type="email" value="a">
 <input type="password">
 <input type="password" value="a">
 <input type="date">
 <input type="month">
 <input type="week">
 <input type="time">
 <input type="datetime-local">
 <input type="number">
 <input type="range">
 <input type="color">
 <input type="checkbox">
 <input type="radio">
 <input type="file">
 <input type="submit">
 <input type="image">
 <input type="reset">
 <input type="button">
 <input type="button" value="a">
 <select><optgroup><option></select>
 <select><optgroup><option>a</select>
 <select multiple></select>
 <select multiple><optgroup>a</optgroup><option>b</option></select>
 <optgroup></optgroup>
 <option></option>
 <button></button>
 <button>a</button>
 <textarea></textarea>
 <textarea>a</textarea>
</div>

</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.input.image.alt.missing",
      "message": "Element “input” is missing required attribute “alt”.",
      "severity": "Warning",
      "span": {
        "byte_end": 744,
        "byte_start": 724,
        "col": 2,
        "line": 31
      }
    },
    {
      "category": "Html",
      "code": "html.input.button.value.nonempty",
      "message": "Element “input” with attribute “type” whose value is “button” must have non-empty attribute “value”.",
      "severity": "Warning",
      "span": {
        "byte_end": 789,
        "byte_start": 768,
        "col": 2,
        "line": 33
      }
    },
    {
      "category": "Html",
      "code": "html.option.empty_without_label",
      "message": "Element “option” without attribute “label” must not be empty.",
      "severity": "Warning",
      "span": {
        "byte_end": 984,
        "byte_start": 975,
        "col": 50,
        "line": 38
      }
    },
    {
      "category": "Html",
      "code": "html.option.empty_without_label",
      "message": "Element “option” without attribute “label” must not be empty.",
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
  "source_name": "html/rendering/non-replaced-elements/form-controls/toggle-display-ref.html"
}
```
