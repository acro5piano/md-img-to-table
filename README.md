# md-img-to-table

Converts markdown image tags into html table tags

# Example

Input (`images.md`) :

```markdown
![image.png](https://image.test/uploads/000000000000000000000000000000000000.png =WxH)
![image.png](https://image.test/uploads/000000000000000000000000000000000001.png =WxH)
![image.png](https://image.test/uploads/000000000000000000000000000000000002.png =WxH)
```

The command:

```bash
cat images.md | md-img-to-table
```

Output:

```html
<table>
  <td>
    <img src="https://image.test/uploads/000000000000000000000000000000000000.png" />
  </td>
  <td>
    <img src="https://image.test/uploads/000000000000000000000000000000000001.png" />
  </td>
  <td>
    <img src="https://image.test/uploads/000000000000000000000000000000000002.png" />
  </td>
</table>
```
