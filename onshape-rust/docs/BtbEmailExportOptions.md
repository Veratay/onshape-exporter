# BtbEmailExportOptions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**email_link** | **bool** | Use 'true' if a link in an email should be sent. | [default to false]
**email_message** | Option<**String**> | Message to send in the email body along with the download link. | [optional]
**email_subject** | Option<**String**> | Subject to send the email with. | [optional][default to User sent you a file exported from Onshape]
**email_to** | **Vec<String>** | List of emails to send the email to. | 
**from_user_id** | **String** | Id of the user who does the export. | 
**password** | Option<**String**> | A password to protect the email with. | [optional][default to false]
**password_required** | Option<**bool**> | Use 'true' if the email should be protected with a password. | [optional][default to false]
**send_copy_to_me** | Option<**bool**> | Use 'true' if email copy should be sent to the user who does the export. | [optional][default to false]
**valid_for_days** | Option<**i32**> | Number of days to keep the link valid for. | [optional][default to 3]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


