/* tslint:disable */
/* eslint-disable */
/**
 * sctgdesk-api-server
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: 0.1.0
 * 
 *
 * NOTE: This class is auto generated by the swagger code generator program.
 * https://github.com/swagger-api/swagger-codegen.git
 * Do not edit the class manually.
 */

 /**
 * 
 *
 * @export
 * @interface UserInfo
 */
export interface UserInfo {

    /**
     * @type {string}
     * @memberof UserInfo
     */
    name: string;

    /**
     * @type {string}
     * @memberof UserInfo
     */
    email?: string | null;

    /**
     * @type {boolean}
     * @memberof UserInfo
     */
    admin: boolean;
}
