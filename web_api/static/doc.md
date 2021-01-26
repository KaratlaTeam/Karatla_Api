
###### code 01 -> data not find ######


1000 -> about account
###### code 1100 -> create account ######

code: 1100
-- type: request
-- function: request create new account
-- api: /api/account/new

code: 1101
-- type: back
-- function: successful create new account

code: 1110
-- type: back
-- function: fail create new account --> code wrong






##### code 1200 -> update account ######

code: 1200
-- type: request
-- function: request update account
-- api: /api/account/update

code: 1201
-- type: back
-- function: successful update account

code: 1210
-- type: back
-- function: fail update account --> code wrong

code: 1211
-- type: back
-- function: fail update account --> did not find update date





##### code 1300 -> delete account #####
-- type: request
-- function: request delete account
-- api: /api/account/delete

code: 1301
-- type: back
-- function: successful delete account

code: 1310
-- type: back
-- function: fail delete account --> code wrong






##### code 1400 -> get account #####
-- type: request
-- function: request get accounts
-- api: /api/account/get

code: 1401
-- type: back
-- function: successful get accounts

code: 1410
-- type: back
-- function: fail get account --> code wrong




##### code 1500 -> login account #####
-- type: request
-- function: request login accounts with 
-- api: /api/account/login
-- code: 01 -> phone login
-- code: 02 -> email login

code: 1501
-- type: back
-- function: successful login accounts

code: 1510
-- type: back
-- function: fail login account --> code wrong

code: 1511
-- type: back
-- function: fail login account --> account login type code wrong

code: 1512
-- type: back
-- function: fail login account --> password wrong



##### code 1600 -> check login account #####
-- type: request
-- function: request check login accounts
-- api: /api/account/check

code: 1601
-- type: back
-- function: successful check login account --> device same

code: 1602
-- type: back
-- function: fail check login account --> device different

code: 1603
-- type: back
-- function: fail check login account --> account different

code: 1610
-- type: back
-- function: fail check login account --> code wrong




##### code 1700 -> logout account #####
-- type: request
-- function: request logout accounts
-- api: /api/account/logout

code: 1701
-- type: back
-- function: successful logout account

code: 1710
-- type: back
-- function: fail logout account --> code wrong




##### code 1800 -> validation code send #####
-- type: request
-- function: send validation code to service

code: 1801
-- type: back
-- function: successful send 

code: 1810
-- type: back
-- function: fail send validation code to service



##### code 1850 -> validation code delete #####
-- type: request
-- function: delete validation code 

code: 1851
-- type: back
-- function: successful delete

code: 1852
-- type: back
-- function: fail delete validation code 

code: 1853
-- type: back
-- function: request code wrong




##### code 1900 -> validation code check #####
-- type: request
-- function: check validation code

code: 1901
-- type: back
-- function: successful check validation code

code: 1910
-- type: back
-- function: fail check validation code --> code wrong

code: 1911
-- type: back
-- function: phone code wrong

code: 1912
-- type: back
-- function: phone already exit



