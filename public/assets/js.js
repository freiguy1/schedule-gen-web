var app = angular.module('ScheduleGenApp', ['ngMaterial', 'ngMdIcons']);

app.controller('AppCtrl', function ($scope) {
    // TEAMS

    $scope.teams = [ 
        { name: "Team 1", id: 0 },
        { name: "Team 2", id: 1 }];

    $scope.removeTeam = function(id) {
        var oldTeams = $scope.teams;
        $scope.teams = [];
        angular.forEach(oldTeams, function(team) {
            if(team.id != id) {
                $scope.teams.push(team);
            }
        });
    };

    $scope.addTeam = function() { 
        $scope.teams.push({ 
            name: '',
            id: $scope.teams[$scope.teams.length - 1].id + 1 
        });
    };


    // LOCATIONS
    $scope.locations = [ 
        { name: "Baseball Field 1", id: 0 },
        { name: "Baseball Field 2", id: 1 }];

    $scope.removeLocation = function(id) {
        var oldLocations = $scope.locations;
        $scope.locations = [];
        angular.forEach(oldLocations, function(location) {
            if(location.id != id) {
                $scope.locations.push(location);
            }
        });
    };

    $scope.addLocation = function() { 
        $scope.locations.push({ 
            name: '',
            id: $scope.locations[$scope.locations.length - 1].id + 1 
        });
    };


});
