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
 * @interface AddUserRequest
 */
export interface AddUserRequest {

    /**
     * @type {string}
     * @memberof AddUserRequest
     */
    name: string;

    /**
     * @type {string}
     * @memberof AddUserRequest
     */
    password: string;

    /**
     * @type {string}
     * @memberof AddUserRequest
     */
    "confirm-password": string;

    /**
     * @type {string}
     * @memberof AddUserRequest
     */
    email: string;

    /**
     * @type {boolean}
     * @memberof AddUserRequest
     */
    is_admin: boolean;

    /**
     * @type {string}
     * @memberof AddUserRequest
     */
    group_name: string;
}
